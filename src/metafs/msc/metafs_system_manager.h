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

#include "metafs_manager_base.h"
#include "metafs_return_code.h"
#include "mfs_state_mgr.h"
#include "msc_req.h"
#include "meta_storage_info.h"

#include <string>

namespace pos
{
class MetaFsSystemManager;
extern MetaFsSystemManager mfsSysMgr;
using MetaFsControlReqHandlerPointer = POS_EVENT_ID (MetaFsSystemManager::*)(MetaFsControlReqMsg&);

class MetaFsSystemManager : public MetaFsManagerBase
{
public:
    MetaFsSystemManager(void);
    ~MetaFsSystemManager(void);
    static MetaFsSystemManager& GetInstance(void);

    const char* GetModuleName(void) override;
    POS_EVENT_ID CheckReqSanity(MetaFsControlReqMsg& reqMsg);

    virtual bool Init(std::string& arrayName, MetaStorageMediaInfoList& mediaInfoList);
    virtual bool Bringup(std::string& arrayName);
    virtual POS_EVENT_ID ProcessNewReq(MetaFsControlReqMsg& reqMsg);

    virtual bool IsMounted(void);

    uint64_t GetEpochSignature(std::string& arrayName);

protected:
    virtual bool _IsSiblingModuleReady(void) override;

private:
    void _InitReqHandler(void);
    void _RegisterReqHandler(MetaFsControlReqType reqType, MetaFsControlReqHandlerPointer handler);
    void _InitiateSystemRecovery(void);

    POS_EVENT_ID _HandleFileSysCreateReq(MetaFsControlReqMsg& reqMsg);
    POS_EVENT_ID _HandleMountReq(MetaFsControlReqMsg& reqMsg);
    POS_EVENT_ID _HandleUnmountReq(MetaFsControlReqMsg& reqMsg);
    POS_EVENT_ID _HandleAddArray(MetaFsControlReqMsg& reqMsg);
    POS_EVENT_ID _HandleRemoveArray(MetaFsControlReqMsg& reqMsg);

    MetaFsControlReqHandlerPointer reqHandler[(uint32_t)MetaFsControlReqType::Max];
    MetaFsStateManager mfsStateMgr; // note that stateMgr shouldn't be called by other modules

    bool isMfsUnmounted;
    bool isTheFirst;

    MetaVolumeMbrMap& mbrMap;
};

extern MetaFsSystemManager& mscTopMgr;
} // namespace pos