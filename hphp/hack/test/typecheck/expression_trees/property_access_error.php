<?hh

<<file:__EnableUnstableFeatures('expression_trees')>>

function test(): void {
  Code`(MyState $x) ==> {
    $x->my_prop = "Hello";
  }`;
}

abstract class MyState {
  public ExampleInt $my_prop;
}
