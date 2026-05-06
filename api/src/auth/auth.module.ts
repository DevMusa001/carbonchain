import { Module } from '@nestjs/common';
import { JwtModule } from '@nestjs/jwt';
import { PassportModule } from '@nestjs/passport';
import { ConfigModule, ConfigService } from '@nestjs/config';
import { AuthService } from './auth.service';
import { AuthController } from './auth.controller';
import { StellarAuthStrategy } from './stellar-auth.strategy';
import { StellarModule } from '../stellar/stellar.module';

@Module({
  imports: [
    PassportModule,
    StellarModule,
    JwtModule.registerAsync({
      imports: [ConfigModule],
      inject: [ConfigService],
      useFactory: (config: ConfigService) => ({
        secret: config.get<string>('JWT_SECRET', 'changeme'),
        signOptions: { expiresIn: '1h' },
      }),
    }),
  ],
  providers: [AuthService, StellarAuthStrategy],
  controllers: [AuthController],
  exports: [AuthService],
})
export class AuthModule {}
