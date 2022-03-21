/*
1. Bashion::supervisor => SupervisorRef
2. SupervisorRef.supervisor =>　SupervisorRef
3. SupervisorRef.children => ChildrenRef
4. Bashion::children => ChildrenRef
5. Supervisor.children => Supervisor

parent.start<T>()
*/

use std::cell::RefCell;
use std::future::Future;
use std::marker::PhantomData;
use std::sync::Arc;
use bastion::Bastion;
use bastion::children::Children;
use bastion::prelude::{BastionContext, Distributor, MessageHandler};
use crate::parent::Parent;
use crate::{StartOpts};
use crate::common::StartInitType;

pub struct GenServer<T, A, R>
    where
        T:  GenServerPart + GenServerMessagePart,
        A: 'static + Send + Clone,
        R: 'static + AsRef<str> + Send + Clone
{
    pub opts: StartOpts<A, R>,
    _marker: PhantomData<T>
}

impl<T, A, R> GenServer<T, A, R>
    where
        T: 'static + Default + GenServerPart + GenServerMessagePart,
        A: 'static + Send + Clone,
        R: 'static + AsRef<str> + Send + Clone
{
    pub fn new(opts: StartOpts<A, R>) -> Self {
        GenServer{
            opts,
            _marker: PhantomData::default()
        }
    }
}

impl<T, A, R> From<GenServer<T, A, R>> for StartInitType
    where
        T: 'static + Default + GenServerPart + GenServerMessagePart,
        A: 'static + Send + Clone,
        R: 'static + AsRef<str> + Send + Clone

{
    fn from(gen_ser: GenServer<T, A, R>) -> Self {
        let init = wrap_children::<T, A, R>(gen_ser.opts);
        StartInitType::GenServer(Box::new(init))
    }
}


pub trait GenServerPart: GenServerLifeCyclePart + Send {
}

/*
GenServer生命周期
*/
pub trait GenServerLifeCyclePart {
    fn init<A, R>(&mut self, _opts: StartOpts<A, R>)
    where
        Self: Sized,
        A: 'static + Send + Clone,
        R: AsRef<str> + Send + Clone
    {

    }

}

// GenServer 消息处理，一般无需手动实现
pub trait GenServerMessagePart {
    fn on_question(&mut self, handler: MessageHandler<()>) -> MessageHandler<()> {
        handler
    }

    fn on_tell(&mut self, handler: MessageHandler<()>) -> MessageHandler<()> {
        handler
    }

    fn on_broadcast(&mut self, handler: MessageHandler<()>) -> MessageHandler<()> {
        handler
    }
}

// fn test(f: Box<dyn FnOnce(Children) -> Children>){
//     Bastion::children(f);
// }

pub(crate) fn wrap_children<T, A, R>(opts: StartOpts<A, R>) -> impl FnOnce(Children) -> Children
where
    A: 'static + Send + Clone,
    T: 'static + Default + GenServerPart + GenServerMessagePart,
    R: AsRef<str> + Send + Clone + 'static
{
    move |children: Children| {
        let children = set_children_opts(children, &opts);
        let children = children.with_exec(move |ctx| {
            gen_server_loop::<T, A, R>(ctx, opts.clone())
        });
        children
    }
}

fn set_children_opts<A, R>(mut children: Children, opts: &StartOpts<A, R>) -> Children
where
    A: 'static + Send + Clone,
    R: AsRef<str> + Send + Clone
{
    let children = if let Some(distributor_name) = &opts.distributor_name {
        children.with_distributor(Distributor::named(distributor_name))
    } else {
        children
    };

    children
}

fn gen_server_loop<T, A, R>(ctx: BastionContext, opts: StartOpts<A, R>) -> impl Future<Output = Result<(), ()>> + Send + 'static
where
    T: Default + GenServerPart + GenServerMessagePart,
    A: 'static + Send + Clone,
    R: AsRef<str> + Send + Clone
{
    async move {
        let mut ser = T::default();
        ser.init(opts);
        loop {
            let handler = MessageHandler::new(ctx.recv().await?);
            let handler = <T as GenServerMessagePart>::on_question(&mut ser, handler);
            let handler = <T as GenServerMessagePart>::on_tell(&mut ser, handler);
            let _handler = <T as GenServerMessagePart>::on_broadcast(&mut ser, handler);
        }
        Ok(())
    }
}