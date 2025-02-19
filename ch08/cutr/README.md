# 주의점

내가 사용하는 cut((GNU coreutils) 9.5)은 UTF8을 지원하지 않는 것 같다.

```bash
cut -c 1-8 tests/inputs/movies1.tsv
```

이 명령의 결과는 다음과 같다.

```text
title ye
The Blue
Les Mis�

```

UTF8를 지원하는 것이 더 낫다고 생각하므로 아래와 같이 변경했다.

```text
title ye
The Blue
Les Misé

```

UTF8 문제로 변경한 파일은 다음과 같다.

- [movies1.tsv.c1-8.out](tests/expected/movies1.tsv.c1-8.out)  
- [movies1.tsv.c8.out](tests/expected/movies1.tsv.c8.out)
