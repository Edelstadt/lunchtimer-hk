import 'package:flutter/material.dart';

darkTheme() => ThemeData(
  brightness: Brightness.dark,
  textTheme: TextTheme(
    bodyText1: TextStyle(
      fontSize: 20.0,
    ),
    bodyText2: TextStyle(
      fontSize: 20.0,
    ),
    headline1: TextStyle(
      fontSize: 24.0,
      fontWeight: FontWeight.w400,
    ),
    headline2: TextStyle(
      fontSize: 24.0,
      fontWeight: FontWeight.w400,
    ),
  ),
);
