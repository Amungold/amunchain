# Miri Known Issues (AmunChain)

## phys004_serialization_stability – Stacked Borrows false positive in `im` crate

- **الاختبار**: `audit_layer01_physics::phys004_serialization_stability`
- **المكتبة المتأثرة**: `im` v15.1.0 (تعتمد على `sized-chunks` v0.6.5)
- **السبب**: دالة `force_copy` في `sized-chunks` تستخدم `ptr::copy` بطريقة يرفضها نموذج Stacked Borrows الحالي.
- **التصنيف**: False positive محتمل أو سلوك غير آمن في المكتبة الخارجية، وليس في كود AmunChain.
- **الإجراء**: تم تخطي هذا الاختبار تحت Miri مع الاحتفاظ به كاملاً للاختبارات العادية.
- **المتابعة**: مراقبة تحديثات `im` و`sized-chunks`؛ الترقية إلى `im` v16+ عند توفره.
- **التاريخ**: 2026-05-27
