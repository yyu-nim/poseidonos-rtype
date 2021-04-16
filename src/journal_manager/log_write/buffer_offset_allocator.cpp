/*
 *   BSD LICENSE
 *   Copyright (c) 2021 Samsung Electronics Corporation
 *   All rights reserved.
 *
 *   Redistribution and use in source and binary forms, with or without
 *   modification, are permitted provided that the following conditions
 *   are met:
 *
 *     * Redistributions of source code must retain the above copyright
 *       notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above copyright
 *       notice, this list of conditions and the following disclaimer in
 *       the documentation and/or other materials provided with the
 *       distribution.
 *     * Neither the name of Intel Corporation nor the names of its
 *       contributors may be used to endorse or promote products derived
 *       from this software without specific prior written permission.
 *
 *   THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 *   "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 *   LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 *   A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 *   OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 *   SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 *   LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 *   DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 *   THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 *   (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 *   OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#include "buffer_offset_allocator.h"

#include <functional>

#include "../checkpoint/log_group_releaser.h"
#include "../config/journal_configuration.h"
#include "src/include/pos_event_id.h"
#include "src/logger/logger.h"

namespace pos
{
BufferOffsetAllocator::BufferOffsetAllocator(void)
: config(nullptr),
  releaser(nullptr),
  nextSeqNumber(UINT32_MAX),
  currentLogGroupId(INT32_MAX),
  maxOffsetPerGroup(UINT64_MAX)
{
}

BufferOffsetAllocator::~BufferOffsetAllocator(void)
{
    for (auto it : statusList)
    {
        delete it;
    }
    statusList.clear();
}

void
BufferOffsetAllocator::Init(LogGroupReleaser* logGroupReleaser,
    JournalConfiguration* journalConfiguration)
{
    releaser = logGroupReleaser;
    config = journalConfiguration;

    int numLogGroups = config->GetNumLogGroups();
    maxOffsetPerGroup = config->GetLogBufferSize() / numLogGroups;

    uint64_t metaPageSize = config->GetMetaPageSize();
    for (int groupId = 0; groupId < numLogGroups; groupId++)
    {
        statusList.push_back(new LogGroupBufferStatus(maxOffsetPerGroup, metaPageSize));
    }

    Reset();
}

void
BufferOffsetAllocator::Reset(void)
{
    for (auto it : statusList)
    {
        it->Reset();
    }

    nextSeqNumber = 0;
    currentLogGroupId = 0;
}

int
BufferOffsetAllocator::AllocateBuffer(uint32_t logSize, OffsetInFile& allocated)
{
    std::lock_guard<std::mutex> lock(allocateLock);

    if (statusList[currentLogGroupId]->GetStatus() == LogGroupStatus::FULL)
    {
        return (int)POS_EVENT_ID::JOURNAL_LOG_GROUP_FULL;
    }

    if (statusList[currentLogGroupId]->GetStatus() == LogGroupStatus::INIT)
    {
        statusList[currentLogGroupId]->SetActive(_GetNextSeqNum());

        POS_TRACE_DEBUG((int)POS_EVENT_ID::JOURNAL_DEBUG,
            "New log group {} is allocated", currentLogGroupId);
    }

    uint64_t offset = 0;
    if (statusList[currentLogGroupId]->TryToAllocate(logSize, offset) == false)
    {
        _TryToSetFull(currentLogGroupId);
        int result = _GetNewActiveGroup();
        if (result != 0)
        {
            return result;
        }

        if (statusList[currentLogGroupId]->TryToAllocate(logSize, offset) == false)
        {
            return (int)POS_EVENT_ID::JOURNAL_LOG_GROUP_FULL;
        }
    }

    // TODO (huijeong.kim) remove sequence number from buffer allocation for log write
    allocated.id = currentLogGroupId;
    allocated.offset = offset;
    allocated.seqNum = statusList[currentLogGroupId]->GetSeqNum();

    return 0;
}

int
BufferOffsetAllocator::_GetNewActiveGroup(void)
{
    int numLogGroups = config->GetNumLogGroups();
    currentLogGroupId = (currentLogGroupId + 1) % numLogGroups;

    if (statusList[currentLogGroupId]->GetStatus() != LogGroupStatus::INIT)
    {
        POS_TRACE_WARN((int)POS_EVENT_ID::JOURNAL_NO_LOG_BUFFER_AVAILABLE,
            "No log buffer available for journal");
        return (int)POS_EVENT_ID::JOURNAL_NO_LOG_BUFFER_AVAILABLE;
    }
    else
    {
        statusList[currentLogGroupId]->SetActive(_GetNextSeqNum());
        POS_TRACE_DEBUG((int)POS_EVENT_ID::JOURNAL_DEBUG,
            "New log group {} is allocated", currentLogGroupId);
        return 0;
    }
}

uint32_t
BufferOffsetAllocator::_GetNextSeqNum(void)
{
    return nextSeqNumber++;
}

void
BufferOffsetAllocator::_TryToSetFull(int id)
{
    bool changedToFullStatus = statusList[id]->TryToSetFull();
    if (changedToFullStatus == true)
    {
        releaser->AddToFullLogGroup(id);

        POS_TRACE_DEBUG((int)POS_EVENT_ID::JOURNAL_LOG_GROUP_FULL,
            "Log group id {} is added to full log group", id);
    }
}

void
BufferOffsetAllocator::LogFilled(int id, MapPageList& dirty)
{
    statusList[id]->LogFilled();
    _TryToSetFull(id);
}

void
BufferOffsetAllocator::LogBufferReseted(int logGroupId)
{
    statusList[logGroupId]->Reset();
}

uint64_t
BufferOffsetAllocator::GetNumLogsAdded(void)
{
    uint64_t numLogsAdded = 0;
    int numLogGroups = config->GetNumLogGroups();

    for (int id = 0; id < numLogGroups; id++)
    {
        numLogsAdded += statusList[id]->GetNumLogsAdded();
    }
    return numLogsAdded;
}

uint64_t
BufferOffsetAllocator::GetNextOffset(void)
{
    uint64_t logGroupOffset = statusList[currentLogGroupId]->GetNextOffset();
    return currentLogGroupId * maxOffsetPerGroup + logGroupOffset;
}

LogGroupStatus
BufferOffsetAllocator::GetBufferStatus(int logGroupId)
{
    return statusList[logGroupId]->GetStatus();
}

uint32_t
BufferOffsetAllocator::GetSequenceNumber(int logGroupId)
{
    return statusList[logGroupId]->GetSeqNum();
}

} // namespace pos