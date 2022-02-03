if [ "$#" -eq 2 ]; then
  for i in $(seq  -f "%02g" "${1-01}" "${2-25}"); do cargo run -q --release --bin day"$i"; done
elif [ "$#" -eq 1 ]; then
  cargo run -q --release --bin "day$(printf "%02d\n" "$1")";
else
  echo "Usage: $0 <01-25> [01-25]"
  echo "Either give a day between 01 and 25 (eq: $0 1)"
  echo "Or provide a range (eg: $0 1 10)"
fi