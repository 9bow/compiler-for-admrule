# admrule-kr-compiler

`legalize-pipeline`의 `.cache/admrule/*.xml`을 입력받아 bare Git 저장소(`admrule-kr`)를 직접 써내는 Rust 컴파일러입니다. 이 프로그램은 법제처 API를 직접 호출하지 않고, 이미 존재하는 캐시만 입력으로 받습니다.

자매 프로젝트: [`compiler-for-precedent`](https://github.com/legalize-kr/compiler-for-precedent) (판례), [`compiler`](https://github.com/legalize-kr/compiler) (법령).

## 현재 상태: 스캐폴드만

이 저장소는 **부트스트랩 단계**입니다. `src/main.rs`는 `unimplemented`를 반환하는 CLI stub이며, 실제 XML → bare repo 변환 로직은 아직 구현되어 있지 않습니다. 의도된 아키텍처는 자매 저장소 [`compiler-for-precedent`](https://github.com/legalize-kr/compiler-for-precedent)의 2-pass 모델(메타데이터 정렬 → 본문 렌더링 병렬화 → 메인 스레드 commit 직렬화)을 미러합니다.

다음 단계에서 구현 예정:

- [ ] `src/xml_parser.rs`: `.cache/admrule/*.xml` 파싱 (발령일자, 행정규칙ID, 행정규칙일련번호, 소관부처명, 행정규칙종류, 조문내용 추출)
- [ ] `src/render.rs`: XML → Markdown 렌더링 + `{기관코드}/{행정규칙종류}/{NFC(행정규칙명)}/본문.md` 경로 계산
- [ ] `src/git_repo.rs`: bare repo writer (compiler-for-precedent의 direct packfile writer 재사용)
- [ ] 별표·서식 첨부는 **metadata-only**로만 frontmatter에 포함 (HARD INVARIANT — 바이너리 미보관)
- [ ] 발령일자 기준 author/committer date, `1970-01-01` 이전 epoch 클램프
- [ ] commit author: `legalize-kr-bot <bot@legalize.kr>` (데이터 repo와 일치)

## 의도된 사용법 (구현 후)

```bash
admrule-kr-compiler <input_cache_dir> [-o <output_git_dir>]
```

```bash
admrule-kr-compiler ../.cache/admrule
git clone ./output.git ./admrule-kr
cd admrule-kr
```

## 의도된 출력 저장소 구조

```
{기관코드}/
  {행정규칙종류}/
    {NFC(행정규칙명)}/
      본문.md
```

예시:
- `1741000/훈령/공공데이터 관리지침/본문.md` (행정안전부)
- `1613000/고시/건축물의 구조기준 등에 관한 규칙/본문.md` (국토교통부)

행정규칙종류: 훈령, 예규, 고시, 공고, 지침, 기타 (OpenAPI `knd` 코드 1~6)

## HARD INVARIANT (변경 불가)

- **첨부(별표·서식) 바이너리는 git tree에 두지 않는다.** LFS도 사용하지 않는다.
- 모든 첨부는 frontmatter `attachments[]` 메타데이터(kind/no/title/source_url/pdf_url/file_type/sha256/size_bytes/fetched_at)로만 보존한다.
- 컴파일러는 `.cache/admrule/*.xml`만 읽고 외부 URL을 follow하지 않는다.
- `target/`·tempdir에 HWP/PDF/이미지 write 금지.

위반 시 integration test가 fail-fast합니다. 자세한 내용은 워크스페이스 루트 [`RESOURCE_ADMRULE.md`](https://github.com/legalize-kr/legalize-kr/blob/main/RESOURCE_ADMRULE.md) §"별표·서식 첨부 처리 정책" 참조.

## 개발

```bash
cargo test
cargo fmt
cargo clippy --no-deps -- -D warnings
cargo build --release
```

### 크로스 컴파일 (macOS)

```bash
brew install filosottile/musl-cross/musl-cross
rustup target add x86_64-unknown-linux-musl aarch64-unknown-linux-musl

cargo build -r --target x86_64-unknown-linux-musl
cargo build -r --target aarch64-unknown-linux-musl
```

&nbsp;

---

*admrule-kr-compiler* is primarily distributed under the terms of both the
[Apache License (Version 2.0)] and the [MIT license].

[MIT license]: LICENSE-MIT
[Apache License (Version 2.0)]: LICENSE-APACHE
