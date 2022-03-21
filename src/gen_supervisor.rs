use std::marker::PhantomData;
use bastion::prelude::{Children, Supervisor};
use crate::parent::Parent;
use crate::{GenServerPart, StartOpts};
use crate::common::{StartInitType, StartResultType};


/* 一个通用的监控进程
 */
pub struct GenSupervisor<T, A, R>
    where
        T: Default,
        A: 'static + Send + Clone,
        R: 'static + AsRef<str> + Send + Clone
{
    pub opts: StartOpts<A, R>,
    _marker: PhantomData<T>
}

impl<T, A, R> GenSupervisor<T, A, R>
    where
        T: Default,
        A: 'static + Send + Clone,
        R: 'static + AsRef<str> + Send + Clone
{
    /*
        使用一个StartOpts生成一个GenServer
    */
    pub fn new(opts: StartOpts<A, R>) -> Self {
        GenSupervisor {
            opts,
            _marker: PhantomData::default()
        }
    }
}

impl<T, A, R> From<GenSupervisor<T, A, R>> for StartInitType
    where
        T: 'static + Default + GenSupervisorPart,
        A: 'static + Send + Clone,
        R: 'static + AsRef<str> + Send + Clone
{
    fn from(gs: GenSupervisor<T, A, R>) -> Self {
        let init = wrap_supervisor::<T, A, R>(gs.opts);
        StartInitType::Supervisor(Box::new(init))
    }
}

pub trait GenSupervisorPart: GenSupervisorLifeCyclePart {

}

pub trait GenSupervisorLifeCyclePart {
    fn init<A, R>(&mut self, opts: StartOpts<A, R>) -> Vec<StartInitType>
        where
            A: 'static + Send + Clone,
            R: 'static + AsRef<str> + Send + Clone,
            Self: Sized
    ;

}

pub(crate) fn wrap_supervisor<T, A, R>(opts: StartOpts<A, R>) -> impl FnOnce(Supervisor) -> Supervisor
where
    T: Default + GenSupervisorPart,
    A: 'static + Send + Clone,
    R: 'static + AsRef<str> + Send + Clone
{
    move |sup| {
        let mut sup = set_options(sup, &opts);
        let mut sup_obj = T::default();
        let children = sup_obj.init(opts.clone());
        for c in children {
            sup.start(c).unwrap();
        }
        sup
    }
}

fn set_options<A, R>(sup: Supervisor, opts: &StartOpts<A, R>) -> Supervisor
where
    A: 'static + Send + Clone,
    R: AsRef<str> + Send + Clone
{
    sup
}
