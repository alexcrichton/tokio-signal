var N = null;var searchIndex = {};
searchIndex["tokio_signal"]={"doc":"Asynchronous signal handling for Tokio","items":[[5,"ctrl_c","tokio_signal","Creates a stream which receives \"ctrl-c\" notifications sent to a process.",N,[[],["iofuture",["iostream"]]]],[5,"ctrl_c_handle","","Creates a stream which receives \"ctrl-c\" notifications sent to a process.",N,[[["handle"]],["iofuture",["iostream"]]]],[0,"unix","","Unix-specific types for signal handling.",N,N],[17,"SIGUSR1","tokio_signal::unix","",N,N],[17,"SIGUSR2","","",N,N],[17,"SIGINT","","",N,N],[17,"SIGTERM","","",N,N],[17,"SIGALRM","","",N,N],[17,"SIGHUP","","",N,N],[17,"SIGPIPE","","",N,N],[17,"SIGQUIT","","",N,N],[17,"SIGTRAP","","",N,N],[3,"Signal","","An implementation of `Stream` for receiving a particular type of signal.",N,N],[11,"new","","Creates a new stream which will receive notifications when the current process receives the signal `signal`.",0,[[["c_int"]],["iofuture",["signal"]]]],[11,"with_handle","","Creates a new stream which will receive notifications when the current process receives the signal `signal`.",0,[[["c_int"],["handle"]],["iofuture",["signal"]]]],[11,"poll","","",0,[[["self"]],["poll",["option","error"]]]],[11,"drop","","",0,[[["self"]]]],[6,"IoFuture","tokio_signal","A future whose error is `io::Error`",N,N],[6,"IoStream","","A stream whose error is `io::Error`",N,N],[11,"into","tokio_signal::unix","",0,[[["self"]],["u"]]],[11,"from","","",0,[[["t"]],["t"]]],[11,"try_from","","",0,[[["u"]],["result"]]],[11,"borrow","","",0,[[["self"]],["t"]]],[11,"get_type_id","","",0,[[["self"]],["typeid"]]],[11,"try_into","","",0,[[["self"]],["result"]]],[11,"borrow_mut","","",0,[[["self"]],["t"]]]],"paths":[[3,"Signal"]]};
initSearch(searchIndex);