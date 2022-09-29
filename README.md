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
$ cargo run --bin poseidonos
   Compiling poseidonos-rtype v0.1.0 (/Users/lefoot/CLionProjects/poseidonos)
    Finished dev [unoptimized + debuginfo] target(s) in 0.66s
     Running `target/debug/poseidonos`
Hello, world!
```
