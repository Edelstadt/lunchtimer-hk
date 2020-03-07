import 'package:flutter/material.dart';
import 'dart:convert';
import 'package:http/http.dart' as http;
import 'package:lunchtimer_ui/restaurant.dart';

class LunchtimerTabView extends StatefulWidget {
  @override
  State<StatefulWidget> createState() {
    return _LunchtimerTabViewState();
  }
}

class _LunchtimerTabViewState extends State<LunchtimerTabView> {
  Future<List<dynamic>> menus;

  @override
  void initState() {
    super.initState();
    menus = fetchMenus();
  }

  Future<List<dynamic>> fetchMenus() async {
//    final response = await http.get("https://jsonplaceholder.typicode.com/photos");
//    if (response.statusCode == 200) {
//      menus = json.decode(response.body) as List;
//    } else {
//      throw Exception('Failed to load photos');
//    }
    const data = '[{"title":"kocour","dishes":[{"label":"dish","price":"120"}],"soups":[{"label":"soup","price":"120"}]}]';

    return json.decode(data) as List;
  }

  @override
  Widget build(BuildContext context) {
    return TabBarView(
      children: <Widget>[
        FutureBuilder(
          future: menus,
          builder: (context, snapshot) {
            if (snapshot.hasData) {
              return Restaurant(snapshot.data);
            } else if (snapshot.hasError) {
              return Text("${snapshot.error}");
            }

            return CircularProgressIndicator();
          },
        ),
        Text('jídla'),
      ],
    );
  }
}
