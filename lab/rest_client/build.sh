cd /Users/foursixnine/Projects/foursixnine.io/cndk8/lab/rest_client
find . -path ./dependencies -prune -false -iwholename '*cmake*' -not -name CMakeLists.txt -delete
rm -rf build
mkdir build
cd build && cmake ..
