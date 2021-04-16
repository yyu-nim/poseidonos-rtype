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

#include "flush_all_user_data_wbt_command.h"

#include <iostream>
#include <string>

#include "src/allocator/i_allocator_wbt.h"
#include "src/allocator_service/allocator_service.h"
#include "src/array/array.h"
#include "src/array_mgmt/array_manager.h"
#include "src/include/array_state_type.h"

namespace pos
{
FlushAllUserDataWbtCommand::FlushAllUserDataWbtCommand(void)
: WbtCommand(FLUSH_USER_DATA, "flush")
{
}

FlushAllUserDataWbtCommand::~FlushAllUserDataWbtCommand(void)
{
}

int
FlushAllUserDataWbtCommand::Execute(Args& argv, JsonElement& elem)
{
    int res = -1;
    ArrayComponents* compo = ArrayMgr::Instance()->_FindArray("");
    if (compo == nullptr)
    {
        return res;
    }
    Array* sysArray = compo->GetArray();
    if (sysArray == nullptr)
    {
        return res;
    }

    ArrayStateType posState = sysArray->GetState();
    if (posState == ArrayStateEnum::NORMAL || posState == ArrayStateEnum::DEGRADED ||
        posState == ArrayStateEnum::REBUILD)
    {
        std::cout << "Start Flush process..." << std::endl;
        // IO Flush & Quiesce
        IAllocatorWbt* iAllocatorWbt = AllocatorServiceSingleton::Instance()->GetIAllocatorWbt("");
        iAllocatorWbt->FlushAllUserdataWBT();
        std::cout << "Stripes finalized..." << std::endl;
        res = 0;
    }
    else
    {
        std::cout << "The state of POS is not proper for flush: " << posState.ToString() << std::endl;
    }

    return res;
}

} // namespace pos