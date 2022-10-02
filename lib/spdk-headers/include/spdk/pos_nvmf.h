/*-
 *   BSD LICENSE
 *
 *   Copyright (c) Samsung Corporation.
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
 *     * Neither the name of Samsung Corporation nor the names of its
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

#ifndef SPDK_POS_NVMF_H_
#define SPDK_POS_NVMF_H_

#include "spdk/stdinc.h"
#include "nvmf_spec.h"
#include "pos.h"

#ifdef __cplusplus
extern "C" {
#endif

#define NR_MAX_NAMESPACE 128
#define NR_MAX_TRANSPORT 4

typedef void (*pos_bdev_delete_callback)(void *cb_arg, int bdeverrno);

/*
 * create pos_bdev disk that will be attached on uNVMf
 */
struct spdk_bdev *spdk_bdev_create_pos_disk(const char *volume_name, uint32_t volume_id,
		const struct spdk_uuid *bdev_uuid, uint64_t num_blocks, uint32_t block_size,
		bool volume_type_in_memory, const char *array_name, uint32_t array_id);

/*
 * delete pos_bdev disk
 */
void spdk_bdev_delete_pos_disk(struct spdk_bdev *bdev, pos_bdev_delete_callback cb_fn,
			       void *cb_arg);

/**
 * Get the NQN ID of the specified subsystem.
 *
 * \param subsystem Subsystem to query.
 *
 * \return NQN ID of the specified subsystem.
 */
uint32_t spdk_nvmf_subsystem_get_id(struct spdk_nvmf_subsystem *subsystem);

/**
 * This fn is used by POS QOS for initializing
 * the subsystem reactor Mapping
 *
 */
void spdk_nvmf_initialize_reactor_subsystem_mapping(void);
/**
 * This fn is used by POS QOS for getting
 * the subsystem reactor Mapping
 *
 */
uint32_t spdk_nvmf_get_reactor_subsystem_mapping(uint32_t reactorId, uint32_t subsystemId);

void spdk_nvmf_update_reactor_subsystem_mapping(struct spdk_nvmf_qpair* qpair);

/**
 *
 */
void spdk_nvmf_configure_pos_qos(bool value);

struct spdk_nvmf_ctrlr *
spdk_nvmf_subsystem_get_first_ctrlr(struct spdk_nvmf_subsystem *subsystem);

struct spdk_nvmf_ctrlr *
spdk_nvmf_subsystem_get_next_ctrlr(struct spdk_nvmf_subsystem *subsystem,
				   struct spdk_nvmf_ctrlr *prev_ctrlr);

char *
spdk_nvmf_subsystem_get_ctrlr_hostnqn(struct spdk_nvmf_ctrlr *ctrlr);

int spdk_nvmf_subsystem_set_pause_state_directly(struct spdk_nvmf_subsystem *subsystem);

void spdk_nvmf_initialize_numa_aware_poll_group(void);

struct spdk_nvmf_poll_group *
spdk_nvmf_get_numa_aware_poll_group(struct spdk_nvmf_tgt *tgt,
				    int numa);

#ifdef __cplusplus
}
#endif

#endif /* SPDK_POS_NVMF_H_ */
