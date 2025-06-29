#[test]
fn test_error_when_self_signed() {
    let res = a2httpc::get("https://self-signed.badssl.com/").send();
    let err = res.err().unwrap();
    println!("{err:?}");
    match err.kind() {
        a2httpc::ErrorKind::Tls(_) => (),
        _ => panic!("wrong error returned!"),
    }
}

#[test]
fn test_accept_invalid_certs_ok_when_self_signed() {
    let res = a2httpc::get("https://self-signed.badssl.com/")
        .danger_accept_invalid_certs(true)
        .send();
    assert!(res.is_ok());
}

#[test]
fn test_accept_invalid_certs_ok_when_wrong_host() {
    let res = a2httpc::get("https://wrong-host.badssl.com/")
        .danger_accept_invalid_certs(true)
        .send();
    assert!(res.is_ok());
}

#[test]
fn test_error_when_wrong_host() {
    let res = a2httpc::get("https://wrong.host.badssl.com/").send();
    let err = res.err().unwrap();
    match err.kind() {
        a2httpc::ErrorKind::Tls(_) => (),
        _ => panic!("wrong error returned!"),
    }
}

#[test]
fn test_accept_invalid_hostnames_error_when_expired() {
    let res = a2httpc::get("https://expired.badssl.com/")
        .danger_accept_invalid_hostnames(true)
        .send();
    let err = res.err().unwrap();
    match err.kind() {
        a2httpc::ErrorKind::Tls(_) => (),
        _ => panic!("wrong error returned!"),
    }
}

#[test]
fn test_accept_invalid_hostnames_ok_when_wrong_host() {
    let res = a2httpc::get("https://wrong.host.badssl.com/")
        .danger_accept_invalid_hostnames(true)
        .send();
    assert!(res.is_ok());
}
