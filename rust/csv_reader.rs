use std::collections::VecDeque; //A double-ended queue
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debig, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point
{
    pub x: usize,//column
    pub y: usize,//row
}

//Constructor
impl Point
{
    pub fb new(x: usize, y: usize) -> self
    {
        Point {x,y}
    }
}

pub type ShortQueue = VecDeque<Point>;






