build:
  cd ./experiment/native/ && cargo build --release

expand:
  cd ./experiment/native/ && cargo expand 

run:
  cd ./experiment/native/ && cargo build --release 
  cd ./experiment/ && dart run
