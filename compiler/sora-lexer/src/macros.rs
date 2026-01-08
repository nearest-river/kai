

#[macro_export]
macro_rules! cfor {
  (($init:stmt;$cond:expr;$step:stmt) $blk:block)=> {
    $init;
    while $cond {
      loop {
        break $block;
      }
      $step;
    }
  };
}






