echo "Checking all requirements..."
if ! command -v "go" >/dev/null; then
  echo "This script requires Go to be installed and on your PATH ..."
  exit 1
fi

if ! command -v "cargo" >/dev/null; then
  echo "This script requires Cargo to be installed and on your PATH ..."
  exit 1
fi

echo "Building dependencies..."

cd libs/libfabric || echo "No such directory: libs/libfabric"
go build -buildmode=c-archive -o ../libfabric.a

cd ..
rm libfabric.h

echo "Done"

