import 'package:flutter/material.dart';
import 'dart:convert';

class Fetcher extends StatefulWidget {
  @override
  State<StatefulWidget> createState() {
    return _FetcherState();
  }
}

class _FetcherState extends State<Fetcher> {
  Future<List<dynamic>> menus;

  Future<List<dynamic>> _fetchMenus() async {
    const data = '[{"title":"kocour","dishes":[{"label":"dish","price":"120"}],"soups":[{"label":"soup","price":"120"}]}]';

    return json.decode(data) as List;
  }

  @override
  void initState() {
    super.initState();
    menus = _fetchMenus();
  }

  @override
  Widget build(BuildContext context) {
    return FutureBuilder(
      future: menus,
      builder: (context, snapshot) {
        if (snapshot.hasData) {
          return Text('ok');
        } else if (snapshot.hasError) {
          return Text("${snapshot.error}");
        }

        return CircularProgressIndicator();
      },
    );
  }
}
