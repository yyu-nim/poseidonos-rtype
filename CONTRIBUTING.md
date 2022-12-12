# Contributing to Poseidon OS R-type

현재 Poseidon OS의 cpp 파일을 rs 파일로 1:1 전환하고 있습니다. 기본적인 cpp 코드의 형식은 그대로 가져가되 문법만 rust로 바꾸는 방식입니다. cpp 의 folder 구조는 그대로 rust의 모듈 구조로 변환합니다. cpp 파일 내 함수, 변수 또한 동일 이름의 rust 함수와 변수로 변환합니다.

## 모듈 추가 방법
src 폴더 내 모든 폴더들은 모듈이 됩니다. 폴더 내 각 파일들 또한 폴더 모듈 내 모듈이 됩니다.  

예를 들어 `src/array` 내 `array_name_policy.cpp`를 rust 파일로 변환하고 싶다면, `src/array`폴더에 `mod.rs`를 추가하여 `array`모듈을 추가하고, 해당 모듈 내에 `array_name_policy` 모듈을 추가, `array_name_policy.rs` 모듈들을 추가합니다.  `array_name_policy.cpp` 파일의 내용은 `array_name_policy.rs` 모듈로 변환됩니다. 

그 결과 생성 및 수정되는 파일들은 `src/lib.rs`, `src/array/mod.rs` 파일과 `src/array/array_name_policy.rs` 파일 입니다. 

```bash
├── src
│   ├── array
│   │     ├── array_name_policy.cpp
│   │     ├── array_name_policy.rs
│   │     ├── mod.rs
│   │     └── ...
│   ├── lib.rs 
...
```

## 테스트 - main 동작 확인
Poseidon OS R-type은 [main-rust.rs](src/main-rust.rs) 에서 시작됩니다. 현재는 시작 시 CLI Server로 `CreateArray` 메시지를 보내고 있습니다. 코드 수정 후 [README.md](README.md)과 동일한 출력이 나오는지 확인합니다.

## 테스트 - unit test
모듈 추가 시 해당 모듈 내 `test` 모듈과 unit test를 함께 추가하여 검증합니다.
