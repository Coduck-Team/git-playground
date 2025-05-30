# git-playground
git2를 사용한 git 클라이언트

## init

```shell
git init
```

`init`을 통해 repository를 생성한다.


## add 

```shell
git add <path>
```

`add`를 통해 변경된 파일을 스테이지에 올린다.

## commit

```shell
git commit <msg>
```

`commit`을 통해 스테이지 된 파일을 기록한다.

## log

```shell
git log
```

`log`를 통해 `commit` 기록을 확인한다.


## push

```shell
git push <remote> <refspec>
```

`push`를 통해 `commit`된 변경사항을 remote에 반영한다. 

## revert
```shell
git revert
```
`revert`를 통해 `commit`을 되돌린다. 

## branch
```shell
git branch
```
`branch`를 통해 브랜치 목록을 보여준다.

<br>

```shell
git branch <branch_name>
```

`branch`를 통해 <branch_name> 브랜치를 생성한다.

<br>

```shell
git branch -d <branch_name>
```

`branch`를 통해 <branch_name> 브랜치를 삭제한다. 
