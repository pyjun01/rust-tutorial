extern crate communicator;

use communicator::ex;
fn main () {
  ex::inner();
  ex::pub_inner();
  ex::modules::inner();
  ex::modules::pub_inner();
  ex::pub_modules::inner();
  ex::pub_modules::pub_inner();

  communicator::client::connect();
}