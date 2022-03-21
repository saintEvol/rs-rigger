pub mod gen_server;
pub mod gen_supervisor;
pub mod parent;
pub mod common;

use std::any::Any;
use bastion::Bastion;
use bastion::prelude::{ChildrenRef, Distributor, MessageHandler, RestartStrategy, SupervisionStrategy};
use crate::gen_server::{GenServerMessagePart, GenServerPart};

#[derive(Clone)]
pub struct StartOpts<A, R>
    where
        A: 'static + Send + Clone,
        R: 'static + AsRef<str> + Send + Clone
{
    pub init_args: Option<A>,
    pub distributor_name: Option<R>,
    // pub children_type: ChildrenType,
    // pub sup_opts: Option<SupervisorOpts>
}

#[derive(Clone)]
pub enum ChildrenType {
    supervisor,
    worker
}

// 监控选项，只有监控者才有此选项
pub struct SupervisorOpts {
    supervision_strategy: Option<SupervisionStrategy>,
    restart_strategy: Option<RestartStrategy>,
}

// pub enum ChildType {
//     Supervisor(Box<dyn Fn() -> (dyn Send + GenServerPart)>),
//     Child(Box<dyn Fn() -> dyn Any>)
// }


pub fn start_gen_server<A, T, R>(opts: StartOpts<A, R>) -> Result<ChildrenRef, ()>
    where
        A: 'static + Send + Clone,
        T: Default + GenServerMessagePart + Send,
        R: AsRef<str> + Send + Clone
{
    Bastion::children(move |children| {
        // let mut ser = DemoGenServer::new();
        // let opts = ser.options();
        //
        let children = if let Some(distributor_name) = opts.distributor_name {
            children.with_distributor(Distributor::named(distributor_name))
        } else {
            children
        };

        // set other options
        let children = children.with_exec(move |ctx| {
            async move {
                let mut ser = T::default();
                // let ser = &mut ser;
                loop {
                    let handler = MessageHandler::new(ctx.recv().await?);
                    let handler = <T as GenServerMessagePart>::on_question(&mut ser, handler);
                    let handler = <T as GenServerMessagePart>::on_tell(&mut ser, handler);
                    let _handler = <T as GenServerMessagePart>::on_broadcast(&mut ser, handler);
                }
                Ok(())
            }
        }
        );

        children
    })
}
