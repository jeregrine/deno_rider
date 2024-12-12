pub fn send_to_pid<T>(
    pid: &rustler::types::LocalPid,
    data: T,
) -> Result<(), rustler::env::SendError>
where
    T: rustler::types::Encoder,
{
    rustler::OwnedEnv::new().send_and_clear(pid, |_env| data)
}
