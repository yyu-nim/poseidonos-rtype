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

#include <cstdint>
#include <string>

namespace pos
{
class JournalConfiguration
{
public:
    JournalConfiguration(void) = default;
    explicit JournalConfiguration(std::string arrayName);
    virtual ~JournalConfiguration(void);

    virtual void Init(void);
    virtual bool IsEnabled(void);
    virtual bool IsDebugEnabled(void);

    virtual int GetNumLogGroups(void);
    virtual uint64_t GetLogBufferSize(void);
    virtual uint64_t GetLogGroupSize(void);
    virtual uint64_t GetMetaPageSize(void);

    virtual void UpdateLogBufferSize(uint64_t size);

protected:
    void _ConfigureLogBufferSize(void);
    uint64_t _GetAlignedSize(uint64_t size);

    bool journalEnabled;
    uint64_t logBufferSizeInConfig;
    uint64_t metaPageSize;
    uint64_t maxPartitionSize;

private:
    void _ReadConfiguration(void);
    bool _IsJournalEnabled(void);
    bool _IsDebugEnabled(void);
    uint64_t _ReadLogBufferSize(void);

    void _ReadMetaFsConfiguration(void);

    bool debugEnabled;

    int numLogGroups;
    uint64_t logBufferSize;
    uint64_t logGroupSize;
    std::string arrayName;

    const uint64_t SIZE_MB = 1024 * 1024;
};

} // namespace pos