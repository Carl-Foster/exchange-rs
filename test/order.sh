wget --header='Content-Type: application/json' --method=post --body-file=$1 http://localhost:8000/contracts/1/orders
wget --header='Content-Type: application/json' --method=post --body-file=$1 http://localhost:8000/contracts/2/orders
