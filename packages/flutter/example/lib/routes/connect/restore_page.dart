import 'package:flutter/material.dart';
import 'package:flutter/services.dart';

class RestorePage extends StatefulWidget {
  final Future Function(String mnemonic) onRestore;

  const RestorePage({super.key, required this.onRestore});

  @override
  State<RestorePage> createState() => _RestorePageState();
}

class _RestorePageState extends State<RestorePage> {
  final _formKey = GlobalKey<FormState>();
  List<TextEditingController> textFieldControllers =
      List<TextEditingController>.generate(12, (_) => TextEditingController());

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(
          backgroundColor: Colors.white,
          actions: [
            IconButton(
              onPressed: () async {
                final clipboardData = await Clipboard.getData('text/plain');
                final clipboardMnemonics = clipboardData?.text?.split(" ");
                if (clipboardMnemonics?.length == 12) {
                  for (var i = 0; i < clipboardMnemonics!.length; i++) {
                    textFieldControllers.elementAt(i).text = clipboardMnemonics.elementAt(i);
                  }
                }
              },
              icon: const Icon(Icons.paste, color: Colors.blue),
            ),
          ],
        ),
        body: Padding(
          padding: const EdgeInsets.symmetric(horizontal: 16.0, vertical: 8.0),
          child: Form(
            key: _formKey,
            child: GridView.builder(
              gridDelegate: SliverGridDelegateWithMaxCrossAxisExtent(
                maxCrossAxisExtent: MediaQuery.of(context).size.width / 2,
                childAspectRatio: 2,
                crossAxisSpacing: 8,
                mainAxisSpacing: 8,
              ),
              itemCount: 12,
              itemBuilder: (BuildContext context, int index) {
                return TextFormField(
                  decoration: InputDecoration(labelText: "${index + 1}"),
                  inputFormatters: [FilteringTextInputFormatter.deny(RegExp(r"\s\b|\b\s"))],
                  validator: (String? value) {
                    if (value == null || value.isEmpty) {
                      return 'Please enter value';
                    }
                    return null;
                  },
                  controller: textFieldControllers[index],
                );
              },
            ),
          ),
        ),
        bottomNavigationBar: Padding(
          padding: EdgeInsets.only(
            bottom: MediaQuery.of(context).viewInsets.bottom + 40.0,
          ),
          child: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              ElevatedButton(
                style: ElevatedButton.styleFrom(
                  backgroundColor: Colors.blue,
                  elevation: 0.0,
                  shape: RoundedRectangleBorder(
                    borderRadius: BorderRadius.circular(8.0),
                  ),
                ),
                onPressed: () async {
                  if (_formKey.currentState?.validate() ?? false) {
                    final mnemonic = textFieldControllers
                        .map((controller) => controller.text.toLowerCase().trim())
                        .toList()
                        .join(" ");
                    widget.onRestore(mnemonic);
                  }
                },
                child: Text(
                  "RESTORE",
                  textAlign: TextAlign.center,
                  style: Theme.of(context).primaryTextTheme.titleMedium,
                  maxLines: 1,
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
