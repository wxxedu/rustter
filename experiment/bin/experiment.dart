import 'package:experiment/experiment.dart' as experiment;

void main(List<String> arguments) {
  // final startAt = DateTime.now();
  // for (var i = 0; i < 1000000; i++) {
  //   experiment.calculate(i, i + 1);
  // }
  // final endAt = DateTime.now();
  // print('FFI Time elapsed: ${endAt.difference(startAt)}');
  // final startAt2 = DateTime.now();
  // for (var i = 0; i < 1000000; i++) {
  //   experiment.calculate2(i, i + 1);
  // }
  // final endAt2 = DateTime.now();
  // print('Dart Time elapsed: ${endAt2.difference(startAt2)}');
  final array = experiment.getArray2();
  experiment.printArray();
  final startAt = DateTime.now();
  for (int i = 0; i < 1000000000; i++) {
    array[0] += 1;
  }
  final endAt = DateTime.now();
  print('FFI Time elapsed: ${endAt.difference(startAt)}');

  List<int> array2 = [0, 1, 2, 3, 4];
  final startAt2 = DateTime.now();
  for (int i = 0; i < 1000000000; i++) {
    array2[0] += 1;
  }
  final endAt2 = DateTime.now();
  print('Dart Time elapsed: ${endAt2.difference(startAt2)}');
}
