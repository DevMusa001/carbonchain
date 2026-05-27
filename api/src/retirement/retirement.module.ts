import { Module } from '@nestjs/common';
import { ConfigModule } from '@nestjs/config';
import { RetirementService } from './retirement.service';
import { RetirementController } from './retirement.controller';
import { CertificateService } from './certificate.service';
import { StellarModule } from '../stellar/stellar.module';

@Module({
  imports: [ConfigModule, StellarModule],
  controllers: [RetirementController],
  providers: [RetirementService, CertificateService],
  exports: [RetirementService, CertificateService],
})
export class RetirementModule {}

