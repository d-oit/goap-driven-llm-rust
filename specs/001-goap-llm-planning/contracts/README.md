# API Contracts

This directory contains API contracts and schemas for the GOAP-Driven LLM system.

## Files

- `api.yaml` - OpenAPI 3.0 specification for the REST API
- `schemas/` - JSON schemas for validation
- `examples/` - Example request/response payloads

## API Endpoints

### Planning

- `POST /plan` - Generate and execute GOAP plan
- `POST /plan/reactive` - Trigger reactive replanning

### Pattern Management

- `GET /patterns` - List cached success patterns
- `GET /patterns/{id}` - Get specific pattern details
- `DELETE /patterns/{id}` - Remove pattern from cache

### Monitoring

- `GET /metrics` - Get performance metrics
- `GET /world-state` - Get current world state

## Usage

Generate client code:
```bash
openapi-generator-cli generate -i contracts/api.yaml -g rust -o src/api/
```

Validate schemas:
```bash
yamale -s contracts/api.yaml data/example_requests.yaml
```

## Integration

The API follows REST conventions with JSON request/response bodies.
All endpoints return structured errors with appropriate HTTP status codes.
