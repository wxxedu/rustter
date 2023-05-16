import 'dart:ffi';

typedef NativeRustAddFunc = Uint32 Function(Uint32, Uint32);
typedef AddFunc = int Function(int, int);

final lib = DynamicLibrary.open('./native/target/release/libnative.dylib');

typedef NativeRustGetArrayFunc = Pointer<Uint32> Function();
typedef GetArrayFunc = Pointer<Uint32> Function();

typedef NativeRustPrintFunc = Void Function();
typedef PrintFunc = void Function();

final addTwo = lib.lookupFunction<NativeRustAddFunc, AddFunc>('add_two_number');
final getArray =
    lib.lookupFunction<NativeRustGetArrayFunc, GetArrayFunc>('get_array');
final printArray =
    lib.lookupFunction<NativeRustPrintFunc, PrintFunc>('print_array');

int calculate(int a, int b) {
  return addTwo(a, b);
}

int calculate2(int a, int b) {
  return a + b;
}

List<int> getArray2() {
  final Pointer<Uint32> array = getArray();
  final List<int> result = array.asTypedList(5);
  return result;
}
