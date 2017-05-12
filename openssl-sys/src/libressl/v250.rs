use libc::{c_int, c_char, c_void, c_long, c_uchar, size_t, c_uint, c_ulong, time_t};

use super::*;

#[repr(C)]
pub struct SSL {
    version: c_int,
    type_: c_int,
    method: *const ::SSL_METHOD,
    rbio: *mut c_void,
    wbio: *mut c_void,
    bbio: *mut c_void,
    rwstate: c_int,
    in_handshake: c_int,
    handshake_func: Option<unsafe extern fn(*mut SSL) -> c_int>,
    pub server: c_int,
    new_session: c_int,
    quiet_shutdown: c_int,
    shutdown: c_int,
    state: c_int,
    rstate: c_int,
    init_buf: *mut c_void,
    init_msg: *mut c_void,
    init_num: c_int,
    init_off: c_int,
    packet: *mut c_uchar,
    packet_length: c_uint,
    s3: *mut c_void,
    d1: *mut c_void,
    read_ahead: c_int,
    msg_callback: Option<unsafe extern fn(c_int, c_int, c_int, *const c_void, size_t, *mut SSL, *mut c_void)>,
    msg_callback_arg: *mut c_void,
    hit: c_int,
    param: *mut c_void,
    cipher_list: *mut stack_st_SSL_CIPHER,
    cipher_list_by_id: *mut stack_st_SSL_CIPHER,
    mac_flags: c_int,
    aead_read_ctx: *mut c_void,
    enc_read_ctx: *mut ::EVP_CIPHER_CTX,
    read_hash: *mut ::EVP_MD_CTX,
    aead_write_ctx: *mut c_void,
    enc_write_ctx: *mut ::EVP_CIPHER_CTX,
    write_hash: *mut ::EVP_MD_CTX,
    cert: *mut c_void,
    sid_ctx_length: c_uint,
    sid_ctx: [c_uchar; ::SSL_MAX_SID_CTX_LENGTH as usize],
    session: *mut ::SSL_SESSION,
    generate_session_id: ::GEN_SESSION_CB,
    verify_mode: c_int,
    verify_callback: Option<unsafe extern fn(c_int, *mut ::X509_STORE_CTX) -> c_int>,
    info_callback: Option<unsafe extern fn(*mut SSL, c_int, c_int)>,
    error: c_int,
    error_code: c_int,
    ctx: *mut ::SSL_CTX,
    debug: c_int,
    verify_result: c_long,
    ex_data: ::CRYPTO_EX_DATA,
    client_CA: *mut stack_st_X509_NAME,
    references: c_int,
    options: c_ulong,
    mode: c_ulong,
    max_cert_list: c_long,
    first_packet: c_int,
    client_version: c_int,
    max_send_fragment: c_uint,
    tlsext_debug_cb: Option<unsafe extern fn(*mut SSL, c_int, c_int, *mut c_uchar, c_int, *mut c_void)>,
    tlsext_debug_arg: *mut c_void,
    tlsext_hostname: *mut c_char,
    servername_done: c_int,
    tlsext_status_type: c_int,
    tlsext_status_expected: c_int,
    tlsext_ocsp_ids: *mut c_void,
    tlsext_ocsp_exts: *mut c_void,
    tlsext_ocsp_resp: *mut c_uchar,
    tlsext_ocsp_resplen: c_int,
    tlsext_ticket_expected: c_int,
    tlsext_ecpointformatlist_length: size_t,
    tlsext_ecpointformatlist: *mut c_uchar,
    tlsext_ellipticcurvelist_length: size_t,
    tlsext_ellipticcurvelist: *mut c_uchar,
    tlsext_session_ticket: *mut c_void,
    tlsext_session_ticket_ext_cb: ::tls_session_ticket_ext_cb_fn,
    tls_session_ticket_ext_cb_arg: *mut c_void,
    tls_session_secret_cb: ::tls_session_secret_cb_fn,
    tls_session_secret_cb_arg: *mut c_void,
    initial_ctx: *mut ::SSL_CTX,
    next_proto_negotiated: *mut c_uchar,
    next_proto_negotiated_len: c_uchar,
    srtp_profiles: *mut c_void,
    srtp_profile: *mut c_void,
    tlsext_heartbeat: c_uint,
    tlsext_hb_pending: c_uint,
    tlsext_hb_seq: c_uint,
    alpn_client_proto_list: *mut c_uchar,
    alpn_client_proto_list_len: c_uint,
    renegotiate: c_int,
}

#[repr(C)]
pub struct SSL_CTX {
    method: *mut c_void,
    cipher_list: *mut c_void,
    cipher_list_by_id: *mut c_void,
    cert_store: *mut c_void,
    sessions: *mut c_void,
    session_cache_size: c_ulong,
    session_cache_head: *mut c_void,
    session_cache_tail: *mut c_void,
    session_cache_mode: c_int,
    session_timeout: c_long,
    new_session_cb: *mut c_void,
    remove_session_cb: *mut c_void,
    get_session_cb: *mut c_void,
    stats: [c_int; 11],
    pub references: c_int,
    app_verify_callback: *mut c_void,
    app_verify_arg: *mut c_void,
    default_passwd_callback: *mut c_void,
    default_passwd_callback_userdata: *mut c_void,
    client_cert_cb: *mut c_void,
    app_gen_cookie_cb: *mut c_void,
    app_verify_cookie_cb: *mut c_void,
    ex_dat: ::CRYPTO_EX_DATA,
    rsa_md5: *mut c_void,
    md5: *mut c_void,
    sha1: *mut c_void,
    extra_certs: *mut c_void,
    comp_methods: *mut c_void,
    info_callback: *mut c_void,
    client_CA: *mut c_void,
    options: c_ulong,
    mode: c_ulong,
    max_cert_list: c_long,
    cert: *mut c_void,
    read_ahead: c_int,
    msg_callback: *mut c_void,
    msg_callback_arg: *mut c_void,
    verify_mode: c_int,
    sid_ctx_length: c_uint,
    sid_ctx: [c_uchar; 32],
    default_verify_callback: *mut c_void,
    generate_session_id: *mut c_void,
    param: *mut c_void,
    quiet_shutdown: c_int,
    max_send_fragment: c_uint,

    #[cfg(not(osslconf = "OPENSSL_NO_ENGINE"))]
    client_cert_engine: *mut c_void,

    tlsext_servername_callback: *mut c_void,
    tlsect_servername_arg: *mut c_void,
    tlsext_tick_key_name: [c_uchar; 16],
    tlsext_tick_hmac_key: [c_uchar; 16],
    tlsext_tick_aes_key: [c_uchar; 16],
    tlsext_ticket_key_cb: *mut c_void,
    tlsext_status_cb: *mut c_void,
    tlsext_status_arg: *mut c_void,
    tlsext_opaque_prf_input_callback: *mut c_void,
    tlsext_opaque_prf_input_callback_arg: *mut c_void,

    next_protos_advertised_cb: *mut c_void,
    next_protos_advertised_cb_arg: *mut c_void,
    next_proto_select_cb: *mut c_void,
    next_proto_select_cb_arg: *mut c_void,

    srtp_profiles: *mut c_void,
}

#[repr(C)]
pub struct SSL_SESSION {
    ssl_version: c_int,
    pub master_key_length: c_int,
    pub master_key: [c_uchar; 48],
    session_id_length: c_uint,
    session_id: [c_uchar; SSL_MAX_SSL_SESSION_ID_LENGTH as usize],
    sid_ctx_length: c_uint,
    sid_ctx: [c_uchar; SSL_MAX_SID_CTX_LENGTH as usize],
    not_resumable: c_int,
    sess_cert: *mut c_void,
    peer: *mut X509,
    verify_result: c_long,
    timeout: c_long,
    time: time_t,
    pub references: c_int,
    cipher: *const c_void,
    cipher_id: c_ulong,
    ciphers: *mut c_void,
    ex_data: ::CRYPTO_EX_DATA,
    prev: *mut c_void,
    next: *mut c_void,
    tlsext_hostname: *mut c_char,
    tlsext_ecpointformatlist_length: size_t,
    tlsext_ecpointformatlist: *mut u8,
    tlsext_ellipticcurvelist_length: size_t,
    tlsext_ellipticcurvelist: *mut u16,
    tlsext_tick: *mut c_uchar,
    tlsext_ticklen: size_t,
    tlsext_tick_lifetime_hint: c_long,
}

#[repr(C)]
pub struct X509_VERIFY_PARAM {
    pub name: *mut c_char,
    pub check_time: time_t,
    pub inh_flags: c_ulong,
    pub flags: c_ulong,
    pub purpose: c_int,
    pub trust: c_int,
    pub depth: c_int,
    pub policies: *mut stack_st_ASN1_OBJECT,
    //pub id: *mut X509_VERIFY_PARAM_ID,
}
