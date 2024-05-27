import 'package:flutter/material.dart';
import 'package:flutter/services.dart';

class MnemonicsDialog extends StatelessWidget {
  final List<String> mnemonics;

  MnemonicsDialog({super.key, required this.mnemonics});

  final textFieldControllers = List<TextEditingController>.generate(12, (_) => TextEditingController());

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: const Text("Mnemonics"),
      content: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          GridView.builder(
            shrinkWrap: true,
            gridDelegate: SliverGridDelegateWithMaxCrossAxisExtent(
              maxCrossAxisExtent: MediaQuery.of(context).size.width / 2,
              childAspectRatio: 2,
              crossAxisSpacing: 8,
              mainAxisSpacing: 8,
            ),
            itemCount: mnemonics.length,
            itemBuilder: (BuildContext context, int index) {
              textFieldControllers[index].text = mnemonics.elementAt(index);
              return TextField(
                readOnly: true,
                controller: textFieldControllers[index],
                decoration: InputDecoration(labelText: "${index + 1}", border: InputBorder.none),
                inputFormatters: [FilteringTextInputFormatter.deny(RegExp(r"\s\b|\b\s"))],
              );
            },
          )
        ],
      ),
      actions: [
        TextButton(
          child: const Text("CLOSE"),
          onPressed: () {
            Navigator.of(context).pop();
          },
        ),
      ],
    );
  }
}
