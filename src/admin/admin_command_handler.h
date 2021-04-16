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

#ifdef _ADMIN_ENABLED
#pragma once

#include "lib/spdk-19.10/lib/nvmf/nvmf_internal.h"
#include "spdk/ibof.h"
#include "src/admin/admin_command_complete_handler.h"
#include "src/event_scheduler/callback.h"
#include "src/event_scheduler/event.h"
#include "src/io_scheduler/io_dispatcher.h"
struct ibof_io;
namespace pos
{
class IArrayInfo;
class IDevInfo;
class IIODispatcher;
class IArrayDevMgr;
class AdminCommandHandler : public Event
{
public:
    AdminCommandHandler(ibof_io* ibofIo, uint32_t originCore, CallbackSmartPtr callback, IArrayInfo* info, IDevInfo* devInfo, IIODispatcher* dispatcher, IArrayDevMgr* arrayDevMgr);
    ~AdminCommandHandler(void);
    bool Execute(void);

private:
    ibof_io* io;
    struct spdk_nvmf_request* req;
    struct spdk_nvme_cmd* cmd;
    uint32_t originCore;
    CallbackSmartPtr callback;
    IArrayInfo* arrayInfo;
    IDevInfo* devInfo;
    IIODispatcher* dispatcher;
    IArrayDevMgr* arrayDevMgr;
};
} // namespace pos
#endif