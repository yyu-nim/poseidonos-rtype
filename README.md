# Poseidon OS R-type

포세이돈 OS의 메모리 안정성을 향상시키기 위한 실험적인 방안의 하나로 RUST 도입을 생각해 볼 수 있다.
C++ 애플리케이션을 RUST로 마이그레이션 하는 다양한 방법이 있을 수 있는데, 본 repo에서는 탑다운 방식으로,
RUST로 작성된 `main()`에서 시작하여, POS에 구현된 기능들을 그대로 가져오거나, Fake를 가져오거나, 
Stub 만드는 방식으로 진행해볼 예정이다. 현재로서는 순수한 호기심/재미/공부의 목적임을 분명히 하고, 
Poseidon OS 과제의 로드맵과는 관련이 없음을 미리 밝혀둔다. 
Production에서의 사용을 권하지 않는다. PR 환영!

각 `.cpp` 파일에 대해 그에 상응하는 `.rs` 파일을 하나씩 만들어갈 예정이다. 
`SPDK`의 경우에는, stub만 만들어 둘 생각이고 추후에 실제 라이브러리와 링크될 수 있도록 
살펴보면 될 것 같고, `NVMe` 디바이스의 경우에는, fake 구현을 해볼 생각이다.


### 실행하는 법
```bash
[2022-10-15T15:48:23Z INFO  poseidonos] Hello, PoseidonOS R-type!
[2022-10-15T15:48:23Z INFO  poseidonos_rtype::spdk_wrapper::spdk] waiting for spdk initialization...
[2022-10-15T15:48:23Z INFO  poseidonos_rtype::spdk_wrapper::spdk::spdk_clib] Invoking start_fn in a new thread...
[2022-10-15T15:48:23Z INFO  poseidonos_rtype::spdk_wrapper::spdk] poseidonos started
[2022-10-15T15:48:23Z INFO  poseidonos_rtype::spdk_wrapper::spdk] spdk_app_start result = 0
[2022-10-15T15:48:24Z INFO  poseidonos_rtype::helper::rpc::spdk_rpc_client] SpdkRpcClient is about to create a transport TCP 64 4096 512
[2022-10-15T15:48:24Z INFO  poseidonos_rtype::helper::rpc::spdk_rpc_client] TODO: send json message to domain socket on /var/tmp/spdk.sock
[2022-10-15T15:48:24Z INFO  poseidonos_rtype::metafs::config::metafs_config_manager] need to build a config
[2022-10-15T15:48:24Z INFO  poseidonos_rtype::main::poseidonos] CLI client is sleeping for 3 seconds...
[2022-10-15T15:48:24Z INFO  poseidonos_rtype::main::poseidonos] CLI server is up...
[2022-10-15T15:48:27Z INFO  poseidonos_rtype::main::poseidonos] CLI client is sending CreateArray msg to CLI server...
[2022-10-15T15:48:27Z INFO  poseidonos_rtype::main::poseidonos] Waiting CLI server to terminate...
[2022-10-15T15:48:27Z INFO  poseidonos_rtype::main::poseidonos] Creating POS array...
[2022-10-15T15:48:27Z INFO  poseidonos_rtype::array_mgmt::array_manager] ArrayManager has been created
[2022-10-15T15:48:27Z INFO  poseidonos_rtype::array_mgmt::array_manager] Creating an array POSArray with devices DeviceSet { nvm: ["uram0"], data: ["data1", "data2", "data3"], spares: ["spare1"] } with meta RAID1 and data RAID5
[2022-10-15T15:48:27Z INFO  poseidonos_rtype::array::state::array_state] ArrayState has been created
[2022-10-15T15:48:27Z INFO  poseidonos_rtype::array_components::array_components] [CREATE_ARRAY_DEBUG_MSG] Creating array component for POSArray
[2022-10-15T15:48:27Z INFO  poseidonos_rtype::array::device::array_device_manager] Importing DeviceSet { nvm: ["uram0"], data: ["data1", "data2", "data3"], spares: ["spare1"] }...
[2022-10-15T15:48:27Z INFO  poseidonos_rtype::array::device::array_device_manager] Exporting devices info with DeviceMeta
[2022-10-15T15:48:27Z INFO  poseidonos_rtype::array::array] [UPDATE_ABR_DEBUG_MSG] Trying to save Array to MBR, name:POSArray, metaRaid:RAID1, dataRaid:RAID5
[2022-10-15T15:48:27Z INFO  poseidonos_rtype::array::array] TODO: _CreatePartitions() ...
[2022-10-15T15:48:27Z INFO  poseidonos_rtype::array::array] [POS_TRACE_ARRAY_CREATED] Array has been created
[2022-10-15T15:48:27Z INFO  poseidonos_rtype::metafs::metafs] Creating MetaFs for POSArray with idx 0
[2022-10-15T15:48:27Z INFO  poseidonos_rtype::volume::volume_manager] Creating VolumeManager for POSArray with idx 0
[2022-10-15T15:48:27Z INFO  poseidonos_rtype::network::nvmf] Creating NVMf for POSArray with idx 0
```
