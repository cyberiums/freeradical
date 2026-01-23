from datetime import datetime, timedelta

sql_template = """
INSERT INTO calendar_efficacy_daily (calendar_system_id, date, avg_match_rate, total_readings, matched_predictions, avg_signal_strength, std_dev, reliability_score)
SELECT 
    o.calendar_system_id,
    DATE('{date}') as date,
    CASE WHEN COUNT(*) > 0 THEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) ELSE 0 END as avg_match_rate,
    COUNT(*)::INTEGER as total_readings,
    SUM(CASE WHEN r.prediction_matched THEN 1 ELSE 0 END)::INTEGER as matched_predictions,
    AVG(r.signal_strength) as avg_signal_strength,
    COALESCE(STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0) as std_dev,
    CASE 
        WHEN AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) > 0 
        THEN 1.0 / (1.0 + COALESCE(
            STDDEV_POP(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END) / 
            NULLIF(AVG(CASE WHEN r.prediction_matched THEN 1.0 ELSE 0.0 END), 0), 0))
        ELSE 0 
    END as reliability_score
FROM oscillator_readings r
JOIN oscillators o ON r.oscillator_id = o.id
WHERE DATE(r.hora_start) = DATE('{date}')
  AND o.calendar_system_id IS NOT NULL
  AND o.is_control = false
GROUP BY o.calendar_system_id
ON CONFLICT (calendar_system_id, date) DO UPDATE SET
    avg_match_rate = EXCLUDED.avg_match_rate,
    total_readings = EXCLUDED.total_readings,
    matched_predictions = EXCLUDED.matched_predictions,
    avg_signal_strength = EXCLUDED.avg_signal_strength,
    std_dev = EXCLUDED.std_dev,
    reliability_score = EXCLUDED.reliability_score;
"""

print("-- Backfill daily aggregations for the past 60 days")
start_date = datetime.utcnow().date()
for i in range(60):
    target_date = start_date - timedelta(days=i)
    print(sql_template.format(date=target_date.isoformat()))
