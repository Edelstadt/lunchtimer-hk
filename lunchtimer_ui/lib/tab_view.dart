import 'package:flutter/material.dart';
import 'package:http/http.dart' as http;

class LunchtimerTabView extends StatefulWidget {
  @override
  State<StatefulWidget> createState() {
    return _LunchtimerTabViewStatus();
  }
}

class _LunchtimerTabViewStatus extends State<LunchtimerTabView> {
  List menus;

  _fetchData() async {
//    final response = await http.get("https://jsonplaceholder.typicode.com/photos");
//    if (response.statusCode == 200) {
//      menus = json.decode(response.body) as List;
//    } else {
//      throw Exception('Failed to load photos');
//    }
    const data = '[{"title":"kocour","dishes":[],"soups":[]}]';
    final decoded = json.decode(data);
    //menus = jsonDecode(data) as List;
  }

  @override
  Widget build(BuildContext context) {
    return TabBarView(
      children: <Widget>[
        Text('restaurace'),
        Text('jídla'),
      ],
    );
  }
}
