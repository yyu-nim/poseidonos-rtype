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

#pragma once

#include <list>

#include "../log/log_event.h"
#include "src/allocator/context_manager/active_stripe_index_info.h"
#include "src/include/address_type.h"

namespace pos
{
class LogHandlerInterface
{
public:
    LogHandlerInterface(void) = default;
    virtual ~LogHandlerInterface(void) = default;

    virtual LogType GetType(void) = 0;
    virtual uint32_t GetSize(void) = 0;
    virtual char* GetData(void) = 0;
    virtual StripeId GetVsid(void) = 0;

    virtual uint32_t GetSeqNum(void) = 0;
    virtual void SetSeqNum(uint32_t num) = 0;
};

class BlockWriteDoneLogHandler : public LogHandlerInterface
{
public:
    BlockWriteDoneLogHandler(void) = default;
    BlockWriteDoneLogHandler(int volId, BlkAddr startRba, uint32_t numBlks,
        VirtualBlkAddr startVsa, int wbIndex, StripeAddr stripeAddr,
        VirtualBlkAddr oldVsa, bool isGC);
    explicit BlockWriteDoneLogHandler(BlockWriteDoneLog& log);
    virtual ~BlockWriteDoneLogHandler(void) = default;

    bool operator==(BlockWriteDoneLogHandler log);

    virtual LogType GetType(void);
    virtual uint32_t GetSize(void);
    virtual char* GetData(void);
    virtual StripeId GetVsid(void);

    virtual uint32_t GetSeqNum(void);
    virtual void SetSeqNum(uint32_t num);

private:
    BlockWriteDoneLog dat;
};

class StripeMapUpdatedLogHandler : public LogHandlerInterface
{
public:
    StripeMapUpdatedLogHandler(void) = default;
    StripeMapUpdatedLogHandler(StripeId vsid, StripeAddr oldAddr, StripeAddr newAddr);
    explicit StripeMapUpdatedLogHandler(StripeMapUpdatedLog& log);
    virtual ~StripeMapUpdatedLogHandler(void) = default;

    bool operator==(StripeMapUpdatedLogHandler log);

    virtual LogType GetType(void);
    virtual uint32_t GetSize(void);
    virtual char* GetData(void);
    virtual StripeId GetVsid(void);

    virtual uint32_t GetSeqNum(void);
    virtual void SetSeqNum(uint32_t num);

private:
    StripeMapUpdatedLog dat;
};

class VolumeDeletedLogEntry : public LogHandlerInterface
{
public:
    VolumeDeletedLogEntry(void) = default;
    explicit VolumeDeletedLogEntry(int volId, uint64_t contextVersion);
    explicit VolumeDeletedLogEntry(VolumeDeletedLog& log);
    virtual ~VolumeDeletedLogEntry(void) = default;

    bool operator==(VolumeDeletedLogEntry log);

    virtual LogType GetType(void);
    virtual uint32_t GetSize(void);
    virtual char* GetData(void);
    virtual StripeId GetVsid(void);

    virtual uint32_t GetSeqNum(void);
    virtual void SetSeqNum(uint32_t num);

private:
    VolumeDeletedLog dat;
};

using LogList = std::list<LogHandlerInterface*>;

} // namespace pos