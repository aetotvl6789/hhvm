//// base-a.php
<?hh

<<file:__EnableUnstableFeatures('require_class')>>

trait A { require class C; }
//// base-b.php
<?hh
class C { use A; }

//// changed-a.php
<?hh

<<file:__EnableUnstableFeatures('require_class')>>

trait A { require class C; }
//// changed-b.php
<?hh
class C {
  public function foo(): void {}
}
