for dir in ./*/; do
  dir_name=$(basename "$dir")
  camel_case_name=$(echo "$dir_name" | awk -F_ '{ for (i=1; i<=NF; i++) $i=toupper(substr($i,1,1)) tolower(substr($i,2)); }1' OFS="")
  echo "$camel_case_name: \"$dir_name\"," >> icons.txt
done
