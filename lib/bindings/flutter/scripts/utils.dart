import 'dart:io';

import 'package:glob/glob.dart';
import 'package:glob/list_local_fs.dart';

extension Globber on String {
  Iterable<String> get glob sync* {
    for (final entity in Glob(this).listSync()) {
      yield entity.path;
    }
  }
}

String get hostTarget {
  final res = Process.runSync('rustc', const ['-vV']);
  return (res.stdout as String)
      .split('\n')
      .firstWhere((line) => line.startsWith('host:'))
      .split(':')
      .last
      .trim();
}

bool fileExists(String path) => File(path).existsSync();

class Observer {
  var fileMap = <String, DateTime?>{};

  String mark(String file) {
    final path = Uri.base.resolve(file).toFilePath();
    final f = File(path);
    fileMap[path] = f.existsSync() ? f.lastModifiedSync() : null;
    return path;
  }

  bool hasChanged(String file) {
    final path = Uri.base.resolve(file).toFilePath();

    if (!fileMap.containsKey(path)) {
      print('❌ Path not marked yet: $path');
      return true;
    }

    final lastModified = fileMap[path];
    if (lastModified == null) {
      print(' Path nonexistent: $path');
      return true;
    }

    return File(path).lastModifiedSync().isAfter(lastModified);
  }
}
