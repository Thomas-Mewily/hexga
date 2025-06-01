/*

Will be in another crate

use crate::*;

pub trait Graph<T> : Get<T>
{
    type NodeId where Self : Get<T>;
}


impl<S,T> Graph<T> for S where S : Get<T>
{
    type NodeId = T;
}

pub trait GraphNode
{

}
*/