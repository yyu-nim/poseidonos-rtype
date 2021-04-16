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

#include <fstream>
#include <iostream>
#include <string>
#include <vector>

#include <rapidjson/document.h>
#include <rapidjson/stringbuffer.h>
#include <rapidjson/writer.h>

#include "metafs.h"
#include "nlohmann/json.hpp"
#include "src/helper/json_helper.h"
#include "src/metafs/include/metafs_return_code.h"
#include "src/metafs/include/mf_dataformat.h"

namespace pos
{
using Args = nlohmann::json;

class WbtMetafsCmdHandler
{
public:
    int DumpFilesList(Args argv);
    int CreateFile(Args argv);
    int OpenFile(Args argv);
    int CloseFile(Args argv);

    int CreateFileSystem(Args argv);
    int MountFileSystem(Args argv);
    int UmountFileSystem(Args argv);

    int ReadFile(Args argv);
    int WriteFile(Args argv);
    int ReadFileAsync(Args argv);
    int WriteFileAsync(Args argv);

    int GetFileSize(Args argv);
    int GetAlignedFileIOSize(Args argv);

    int DumpInodeInfo(Args argv);
    int SetInodeInfo(Args argv);
    int64_t GetMaxFileSize(Args argv);
    int GetFileChecksum(Args argv);

    int GetCurrentSystemState(void);
    int GetNextSystemState(void);
    int CorruptFileSystemMBR(void);
    int CorruptFileSystemSignature(void);
    int SetAllBitsInFDInUse(void); // What happens if all fileDesc Are Used ?
    int GetAllBitsInFDInUse(Args argv);
    int GetTotalFreeInodes(void);
    int SetAllInodeEntryInUseBitmap(void); // All Inodes are Used ?
    int GetAllInodeEntryInUseBitmap(Args argv);
    int SetupMetaFioTest(Args argv);

private:
    int _WriteBufferInFile(std::string fileName, const char* buffer, int bufferSize);
    int _ReadFileInBuffer(std::string fileName, char** buffer, uint32_t& fileSize);
    void _DumpFilesInfoToJsonElement(MetaFileInfoDumpCxt data, JsonArray& element);
    void _DumpInodeInfoToJson(MetaFileInodeDumpCxt *data, JsonElement& element);
    void _SetValuesInMetaFileInode(MetaFileInodeInfo& metaFileInode, rapidjson::Value& inodeData);
};
} // namespace pos