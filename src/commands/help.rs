pub fn git_help() {
    println!("명령어");
    println!("init: .git 생성");
    println!("add <path>: 변경 사항을 스테이지에 올림");
    println!("commit <msg>: 변경 사항을 기록");
    println!("push <remote> <refspec>: 기록된 사항을 remote에 전송");
    println!("revert <commit_id>: commit된 기록을 롤백");
    println!("reset <file_path> <soft|mixed|hard>: file의 변경 상태를 이전으로 변경");
    println!("log: 로그 출력");
    println!("branch: 브랜치 출력");
    println!("checkout <name>: <name> 브랜치로 체크아웃");
    println!("q: 종료")
}
