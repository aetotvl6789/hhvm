<?hh

<<__EntryPoint>>
function main() {
  $a = array("10" => "string!", 10 => "number!");
  $b = array(10 => "string!", "10" => "number!");

  var_dump((array)$a);
  var_dump((array)$b);
}
