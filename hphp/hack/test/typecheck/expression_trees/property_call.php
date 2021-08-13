<?hh

<<file:__EnableUnstableFeatures('expression_trees')>>

function test(): void {
  Code`(MyState $x) ==> {
    return ($x->my_prop)(1);
  }`;
}

abstract class MyState {
  public (function(ExampleInt): ExampleInt) $my_prop;
}
