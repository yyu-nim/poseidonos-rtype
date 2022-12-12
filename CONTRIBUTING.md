# Contributing to Poseidon OS R-type

현재 Poseidon OS의 cpp 파일을 rs 파일로 1:1 전환하고 있습니다. 기본적인 cpp 코드의 형식은 그대로 가져가되 문법만 rust로 바꾸는 방식입니다. cpp 의 folder 구조는 그대로 rust의 모듈 구조로 변환합니다. cpp 파일 내 class, 함수 또한 동일 이름의 rust struct와 함수로 변환합니다.

## 모듈 추가하기
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

## cpp class를 rust struct로 변환하기 
cpp class는 동일 이름의 rust struct로 변환합니다. 

예를 들어  `array.h`에 정의된 class `Array`는 `array.rs` 내 동일 이름의 rust struct로 변환됩니다. `array.cpp`에 정의된 class `Array`의 method들은 rust struct `Array`의 method로 변환됩니다. 

Interface로 사용되고 있는 대문자 I로 시작되는 이름을 가진 cpp class들은 rust의 trait으로 변환합니다.

다음 예에서 `Array` class가 상속받고 있는 `IMountSequence`는 동일 이름의 rust trait으로 변환됩니다.

Class의 생성자는 연관함수인 `new` 함수로 구현합니다. Class의 소멸자는 필요하다면 `Drop` trait으로 구현합니다.

```cpp
// array.h
class Array: public IMountSequence
{
public:
	Array(...);
	virtual int Init(void) override;
	virtual int Dispose(void) override;
	...
	void SetPreferences(bool isWT);
	void SetTargetAddress(string targetAddress);
	...

private:
	int _LoadImpl(void);
	...
	
	string name_;
	unsigned int index_ = 0;
	...
};
```

```rust
// array.rs
pub struct Array {
    name: String,
    index: u32,
    ...
}

impl IMountSequence for Array {
    fn Init(&self) -> i32 {
        ...
    }
    fn Dispose(&self) {}
    ...
}

impl Array {
    fn new() -> Self {
        Array {
            ...
        }
    }
    fn _LoadImpl(&self) -> i32 {
        ...
    }
}
```

## Naming Rule
Class 이름과 Method 이름은 cpp의 naming rule 그대로 camel case를 따릅니다. `Array` class는 `Array` struct로, `_LoadImpl` 함수 또한 그대로 `_LoadImpl` 함수로 변환합니다.

Method 내 local variable은 표준 rust naming 방식 대로 snake case로 정합니다. `isWriteThrough`는 `is_write_through`로 변환합니다. 

## 테스트 - main 동작 확인
Poseidon OS R-type은 [main-rust.rs](src/main-rust.rs) 에서 시작됩니다. 현재는 시작 시 CLI Server로 `CreateArray` 메시지를 보내고 있습니다. 코드 수정 후 [README.md](README.md)과 동일한 출력이 나오는지 확인합니다.

## 테스트 - unit test
모듈 추가 시 해당 모듈 내 `test` 모듈과 unit test를 함께 추가하여 검증합니다.
