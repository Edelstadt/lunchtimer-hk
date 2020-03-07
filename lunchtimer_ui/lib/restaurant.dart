import 'package:flutter/material.dart';
import 'package:lunchtimer_ui/food_table.dart';

class Restaurant extends StatefulWidget {
  List<dynamic> menus;

  Restaurant(List<dynamic> menus) {
    this.menus = menus;
  }

  @override
  State<StatefulWidget> createState() {
    return _RestaurantState();
  }
}

class _RestaurantState extends State<Restaurant> {
  int index = 0;

  @override
  Widget build(BuildContext context) {
    final menus = widget.menus;
    if (menus.length < index) {
      return Text('');
    }

    final menu = widget.menus[this.index];

    return Center(
      child: Column(
        children: <Widget>[
          Padding(
            padding: EdgeInsets.all(20),
            child: Text(
              menu['title'],
              style: Theme.of(context).textTheme.headline2,
            ),
          ),
          FoodTable(menu),
        ],
      ),
    );
  }

}
