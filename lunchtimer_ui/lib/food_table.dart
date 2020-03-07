import 'package:flutter/material.dart';

class FoodTable extends StatelessWidget {
  Map<String, dynamic> menu;

  FoodTable(Map<String, dynamic> menu) {
    this.menu = menu;
  }

  @override
  Widget build(BuildContext context) {
    return Column(
      children: <Widget>[
        Table(
          children: <TableRow>[
            TableRow(
              children: [
                Text('1'),
                Text('12'),
                Text('13'),
              ],
            ),
          ],
        ),
      ],
    );
  }
}
