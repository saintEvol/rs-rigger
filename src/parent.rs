use bastion::Bastion;
use bastion::children_ref::ChildrenRef;
use bastion::prelude::{Supervisor, SupervisorRef};
use crate::common::{StartInitType, StartResultType};
use crate::gen_server::wrap_children;
use crate::gen_supervisor::wrap_supervisor;
use crate::{GenServerMessagePart, GenServerPart, StartOpts};

pub enum ChildrenResult {
    SupervisorRef(SupervisorRef),
    // Supervisor(Supervisor),
    ChildrenRef(ChildrenRef)
}

//
pub struct Root;

pub trait Parent {
    fn start(&mut self, init: impl Into<StartInitType>) -> Result<StartResultType, ()>;
}

impl Parent for Root {
    fn start(&mut self, init: impl Into<StartInitType>) -> Result<StartResultType, ()> {
        match init.into() {
            StartInitType::GenServer(init) => {
                match Bastion::children(init)  {
                    Ok(childref) => Ok(StartResultType::ChildrenRef(childref)),
                    Err(e) => Err(e)
                }
            },
            StartInitType::Supervisor(init) =>
                match Bastion::supervisor(init) {
                    Ok(sup_ref) => Ok(StartResultType::SupervisorRef(sup_ref)),
                    Err(e) => Err(())
                }
        }
    }
}

impl Parent for SupervisorRef {
    fn start(&mut self, init: impl Into<StartInitType>) -> Result<StartResultType, ()> {
        match init.into() {
            StartInitType::GenServer(init) => {
                match self.children(init) {
                    Ok(child_ref) => Ok(StartResultType::ChildrenRef(child_ref)),
                    Err(err) => Err(err)
                }
            },
            StartInitType::Supervisor(init) => {
                match self.supervisor(init) {
                    Ok(sup_ref) => Ok(StartResultType::SupervisorRef(sup_ref)),
                    Err(err) => Err(err)
                }
            }
        }
    }
}

impl Parent for Supervisor {
    fn start(&mut self, init: impl Into<StartInitType>) -> Result<StartResultType, ()> {
        match init.into() {
            StartInitType::GenServer(init) => {
                let r = self.children_ref(init);
                Ok(StartResultType::ChildrenRef(r))
            },
            StartInitType::Supervisor(init) => {
                let r = self.supervisor_ref(init);
                Ok(StartResultType::SupervisorRef(r))
            }
        }
    }
}

