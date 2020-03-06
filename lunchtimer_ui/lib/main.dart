import 'package:flutter/material.dart';
import 'package:flutter/widgets.dart';

void main() => runApp(Lunchtimer());

class Lunchtimer extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Lunchtimer',
      themeMode: ThemeMode.dark,
      darkTheme: ThemeData(
        brightness: Brightness.dark,
      ),
      home: Scaffold(
        appBar: AppBar(
          title: Text('Welcome to Flutter'),
        ),
        body: Center(
          child: Text(''),
        ),
      ),
    );
  }
}
