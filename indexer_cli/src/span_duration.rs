use std::time::Instant;

use tracing::{field::FieldSet, span::Attributes, Event, Id, Subscriber, Value};
use tracing_subscriber::{layer::Context, registry::LookupSpan, Layer};

pub struct SpanDurationLayer;

impl<S> Layer<S> for SpanDurationLayer
where
    S: Subscriber,
    S: for<'lookup> LookupSpan<'lookup>,
{
    fn on_new_span(&self, _attrs: &Attributes<'_>, id: &Id, ctx: Context<'_, S>) {
        let span = ctx.span(id).unwrap();

        span.extensions_mut().insert(Timing {
            started_at: Instant::now(),
        });
    }

    fn on_close(&self, id: Id, ctx: Context<'_, S>) {
        let span = ctx.span(&id).unwrap();

        if ctx.enabled(span.metadata()) && span.fields().field("duration").is_some() {
            let started_at = span.extensions().get::<Timing>().unwrap().started_at;
            let duration = (Instant::now() - started_at).as_micros() as f64;
            let duration = format!("{} ms", duration / 1000_f64);
            let fields = FieldSet::new(&["duration"], span.metadata().callsite());

            let values = &[(
                &fields.field("duration").unwrap(),
                Some(&duration as &dyn Value),
            )];

            let values = fields.value_set(values);
            let ev = Event::new_child_of(id, span.metadata(), &values);

            ctx.event(&ev);
        }
    }
}

struct Timing {
    started_at: Instant,
}
