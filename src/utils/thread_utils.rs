use ::libc;
extern "C" {
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn WebPSafeCalloc(nmemb: uint64_t, size: size_t) -> *mut libc::c_void;
    fn WebPSafeFree(ptr: *mut libc::c_void);
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_cond_init(
        __cond: *mut pthread_cond_t,
        __cond_attr: *const pthread_condattr_t,
    ) -> libc::c_int;
    fn pthread_cond_destroy(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint64_t = libc::c_ulong;
pub type uint64_t = __uint64_t;
pub type WebPWorkerStatus = libc::c_uint;
pub const WORK: WebPWorkerStatus = 2;
pub const OK: WebPWorkerStatus = 1;
pub const NOT_OK: WebPWorkerStatus = 0;
pub type WebPWorkerHook = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPWorker {
    pub impl_: *mut libc::c_void,
    pub status_: WebPWorkerStatus,
    pub hook: WebPWorkerHook,
    pub data1: *mut libc::c_void,
    pub data2: *mut libc::c_void,
    pub had_error: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPWorkerInterface {
    pub Init: Option::<unsafe extern "C" fn(*mut WebPWorker) -> ()>,
    pub Reset: Option::<unsafe extern "C" fn(*mut WebPWorker) -> libc::c_int>,
    pub Sync_0: Option::<unsafe extern "C" fn(*mut WebPWorker) -> libc::c_int>,
    pub Launch: Option::<unsafe extern "C" fn(*mut WebPWorker) -> ()>,
    pub Execute: Option::<unsafe extern "C" fn(*mut WebPWorker) -> ()>,
    pub End: Option::<unsafe extern "C" fn(*mut WebPWorker) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WebPWorkerImpl {
    pub mutex_: pthread_mutex_t,
    pub condition_: pthread_cond_t,
    pub thread_: pthread_t,
}
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [libc::c_char; 48],
    pub __align: libc::c_longlong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_cond_s {
    pub __wseq: __atomic_wide_counter,
    pub __g1_start: __atomic_wide_counter,
    pub __g_refs: [libc::c_uint; 2],
    pub __g_size: [libc::c_uint; 2],
    pub __g1_orig_size: libc::c_uint,
    pub __wrefs: libc::c_uint,
    pub __g_signals: [libc::c_uint; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __atomic_wide_counter {
    pub __value64: libc::c_ulonglong,
    pub __value32: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_condattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
unsafe extern "C" fn ThreadLoop(mut ptr: *mut libc::c_void) -> *mut libc::c_void {
    let worker: *mut WebPWorker = ptr as *mut WebPWorker;
    let impl_0: *mut WebPWorkerImpl = (*worker).impl_ as *mut WebPWorkerImpl;
    let mut done: libc::c_int = 0 as libc::c_int;
    while done == 0 {
        pthread_mutex_lock(&mut (*impl_0).mutex_);
        while (*worker).status_ as libc::c_uint == OK as libc::c_int as libc::c_uint {
            pthread_cond_wait(&mut (*impl_0).condition_, &mut (*impl_0).mutex_);
        }
        if (*worker).status_ as libc::c_uint == WORK as libc::c_int as libc::c_uint {
            ((*WebPGetWorkerInterface()).Execute)
                .expect("non-null function pointer")(worker);
            (*worker).status_ = OK;
        } else if (*worker).status_ as libc::c_uint
            == NOT_OK as libc::c_int as libc::c_uint
        {
            done = 1 as libc::c_int;
        }
        pthread_mutex_unlock(&mut (*impl_0).mutex_);
        pthread_cond_signal(&mut (*impl_0).condition_);
    }
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn ChangeState(
    worker: *mut WebPWorker,
    mut new_status: WebPWorkerStatus,
) {
    let impl_0: *mut WebPWorkerImpl = (*worker).impl_ as *mut WebPWorkerImpl;
    if impl_0.is_null() {
        return;
    }
    pthread_mutex_lock(&mut (*impl_0).mutex_);
    if (*worker).status_ as libc::c_uint >= OK as libc::c_int as libc::c_uint {
        while (*worker).status_ as libc::c_uint != OK as libc::c_int as libc::c_uint {
            pthread_cond_wait(&mut (*impl_0).condition_, &mut (*impl_0).mutex_);
        }
        if new_status as libc::c_uint != OK as libc::c_int as libc::c_uint {
            (*worker).status_ = new_status;
            pthread_mutex_unlock(&mut (*impl_0).mutex_);
            pthread_cond_signal(&mut (*impl_0).condition_);
            return;
        }
    }
    pthread_mutex_unlock(&mut (*impl_0).mutex_);
}
unsafe extern "C" fn Init(worker: *mut WebPWorker) {
    memset(
        worker as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<WebPWorker>() as libc::c_ulong,
    );
    (*worker).status_ = NOT_OK;
}
unsafe extern "C" fn Sync(worker: *mut WebPWorker) -> libc::c_int {
    ChangeState(worker, OK);
    return ((*worker).had_error == 0) as libc::c_int;
}
unsafe extern "C" fn Reset(worker: *mut WebPWorker) -> libc::c_int {
    let mut current_block: u64;
    let mut ok: libc::c_int = 1 as libc::c_int;
    (*worker).had_error = 0 as libc::c_int;
    if ((*worker).status_ as libc::c_uint) < OK as libc::c_int as libc::c_uint {
        let impl_0: *mut WebPWorkerImpl = WebPSafeCalloc(
            1 as libc::c_int as uint64_t,
            ::core::mem::size_of::<WebPWorkerImpl>() as libc::c_ulong,
        ) as *mut WebPWorkerImpl;
        (*worker).impl_ = impl_0 as *mut libc::c_void;
        if ((*worker).impl_).is_null() {
            return 0 as libc::c_int;
        }
        if pthread_mutex_init(&mut (*impl_0).mutex_, 0 as *const pthread_mutexattr_t)
            != 0
        {
            current_block = 4250664136965530248;
        } else if pthread_cond_init(
            &mut (*impl_0).condition_,
            0 as *const pthread_condattr_t,
        ) != 0
        {
            pthread_mutex_destroy(&mut (*impl_0).mutex_);
            current_block = 4250664136965530248;
        } else {
            pthread_mutex_lock(&mut (*impl_0).mutex_);
            ok = (pthread_create(
                &mut (*impl_0).thread_,
                0 as *const pthread_attr_t,
                Some(
                    ThreadLoop
                        as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
                ),
                worker as *mut libc::c_void,
            ) == 0) as libc::c_int;
            if ok != 0 {
                (*worker).status_ = OK;
            }
            pthread_mutex_unlock(&mut (*impl_0).mutex_);
            if ok == 0 {
                pthread_mutex_destroy(&mut (*impl_0).mutex_);
                pthread_cond_destroy(&mut (*impl_0).condition_);
                current_block = 4250664136965530248;
            } else {
                current_block = 12124785117276362961;
            }
        }
        match current_block {
            12124785117276362961 => {}
            _ => {
                WebPSafeFree(impl_0 as *mut libc::c_void);
                (*worker).impl_ = 0 as *mut libc::c_void;
                return 0 as libc::c_int;
            }
        }
    } else if (*worker).status_ as libc::c_uint > OK as libc::c_int as libc::c_uint {
        ok = Sync(worker);
    }
    return ok;
}
unsafe extern "C" fn Execute(worker: *mut WebPWorker) {
    if ((*worker).hook).is_some() {
        (*worker).had_error
            |= (((*worker).hook)
                .expect("non-null function pointer")((*worker).data1, (*worker).data2)
                == 0) as libc::c_int;
    }
}
unsafe extern "C" fn Launch(worker: *mut WebPWorker) {
    ChangeState(worker, WORK);
}
unsafe extern "C" fn End(worker: *mut WebPWorker) {
    if !((*worker).impl_).is_null() {
        let impl_0: *mut WebPWorkerImpl = (*worker).impl_ as *mut WebPWorkerImpl;
        ChangeState(worker, NOT_OK);
        pthread_join((*impl_0).thread_, 0 as *mut *mut libc::c_void);
        pthread_mutex_destroy(&mut (*impl_0).mutex_);
        pthread_cond_destroy(&mut (*impl_0).condition_);
        WebPSafeFree(impl_0 as *mut libc::c_void);
        (*worker).impl_ = 0 as *mut libc::c_void;
    }
}
static mut g_worker_interface: WebPWorkerInterface = unsafe {
    {
        let mut init = WebPWorkerInterface {
            Init: Some(Init as unsafe extern "C" fn(*mut WebPWorker) -> ()),
            Reset: Some(Reset as unsafe extern "C" fn(*mut WebPWorker) -> libc::c_int),
            Sync_0: Some(Sync as unsafe extern "C" fn(*mut WebPWorker) -> libc::c_int),
            Launch: Some(Launch as unsafe extern "C" fn(*mut WebPWorker) -> ()),
            Execute: Some(Execute as unsafe extern "C" fn(*mut WebPWorker) -> ()),
            End: Some(End as unsafe extern "C" fn(*mut WebPWorker) -> ()),
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C" fn WebPSetWorkerInterface(
    winterface: *const WebPWorkerInterface,
) -> libc::c_int {
    if winterface.is_null() || ((*winterface).Init).is_none()
        || ((*winterface).Reset).is_none() || ((*winterface).Sync_0).is_none()
        || ((*winterface).Launch).is_none() || ((*winterface).Execute).is_none()
        || ((*winterface).End).is_none()
    {
        return 0 as libc::c_int;
    }
    g_worker_interface = *winterface;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn WebPGetWorkerInterface() -> *const WebPWorkerInterface {
    return &mut g_worker_interface;
}
