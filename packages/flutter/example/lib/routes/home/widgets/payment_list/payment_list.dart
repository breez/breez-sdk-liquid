import 'package:flutter/material.dart';
import 'package:flutter_breez_liquid/flutter_breez_liquid.dart';
import 'package:flutter_breez_liquid_example/routes/home/widgets/payment_list/widgets/payment_item.dart';

class PaymentList extends StatelessWidget {
  final Future Function() onRefresh;
  final Stream<List<Payment>> paymentsStream;

  const PaymentList({super.key, required this.paymentsStream, required this.onRefresh});

  @override
  Widget build(BuildContext context) {
    return StreamBuilder<List<Payment>>(
      stream: paymentsStream,
      builder: (context, paymentsSnapshot) {
        if (paymentsSnapshot.hasError) {
          return Text('Error: ${paymentsSnapshot.error}');
        }

        if (!paymentsSnapshot.hasData) {
          return const Center(child: Text('Loading...'));
        }

        if (paymentsSnapshot.requireData.isEmpty) {
          return Center(
            child: Text(
              'You are ready to receive funds.',
              style: Theme.of(context).textTheme.bodyMedium,
            ),
          );
        }

        List<Payment> paymentList = paymentsSnapshot.data!.reversed.toList();

        return RefreshIndicator(
          onRefresh: () async {
            debugPrint("Pulled to refresh");
            return await onRefresh();
          },
          child: ListView.builder(
            itemCount: paymentList.length,
            shrinkWrap: true,
            primary: true,
            itemBuilder: (BuildContext context, int index) => PaymentItem(item: paymentList[index]),
          ),
        );
      },
    );
  }
}
